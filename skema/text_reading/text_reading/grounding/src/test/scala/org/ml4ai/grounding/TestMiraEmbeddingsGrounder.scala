package org.ml4ai.grounding

import com.typesafe.config.ConfigFactory
import org.json4s.JsonDSL.boolean2jvalue
import org.ml4ai.grounding.common.utils.{BuildUtils, Sourcer, Test}

import java.io.File
import java.nio.file.Path
import org.scalatest.OptionValues._

import java.io.{BufferedWriter, FileWriter}



class TestMiraEmbeddingsGrounder extends Test {

  def getTextFromResource(path: String): String = {
    val source = Sourcer.sourceFromResource(path)
    val text = source.mkString

    source.close
    text
  }

  // Lazily load the grounder. We assume it's state and behavior is immutable
  // So we can build it once and reuse it as necessary in the suite
  //lambda = 1,10,100
  val lambda = 10

    val miraEmbeddingsGrounder: MiraEmbeddingsGrounder = {
      val config = ConfigFactory.load().getConfig("Grounding")
      val ontologyPath = config.getString("ontologyPath")
      val embeddingsPath = config.getString("embeddingsPath")

    MiraEmbeddingsGrounder(new File(ontologyPath), new File(embeddingsPath), lambda)
  }



  def correctGrounding(text: String, groundingID: String): Unit = {
    it should f"ground $text to the grounding concept with id $groundingID" in {
      val groundedConcept = miraEmbeddingsGrounder ground text
      groundedConcept.value.id should be(groundingID)
    }
  }

  behavior of "Exact matches"
  // These are sanity checks. The input text is exactly the description of the entity, so it should match perfectly with the corresponding concept
  it should behave like correctGrounding("COVID-19", "doid:0080600")
  it should behave like correctGrounding("cell part", "caro:0000014")
  // This is a slightly harder entity to ground
  it should behave like correctGrounding("junctional epidermolysis bullosa non-Herlitz type", "doid:0060738")

  behavior of "Synonym matches"
  // These matches are to synonyms. Are not exactly the same, but should be handled correctly by the grounding algorithm
  it should behave like correctGrounding("junctional epidermolysis bullosa generalisata mitis", "doid:0060738")
  it should behave like correctGrounding("covid19", "doid:0080600")
  it should behave like correctGrounding("s-block compounds", "chebi:33674")

  behavior of "Accuracy of the matches"
  // This is the main test, where we are measuring the accuracy of the grounding according to a set of test gronding queries
  it should "achieve at least 70% accuracy" in {
    val groundingTargets = {
      val targets = {
        // Drop the first line that is the header
        getTextFromResource("/grounding_tests.tsv").split("\n").drop(1) map {
          l =>
            val tokens = l.split("\t")
            (tokens(0), tokens(1))
        }
      }

      val predictions =
        for {
          (text, groundingId) <- targets
        } yield miraEmbeddingsGrounder.ground(text) match {
          case Some(concept) => concept.id == groundingId
          case None => false
        }

      val predictions1 =
        for {
          (text, groundingId) <- targets
        } yield miraEmbeddingsGrounder.ground(text) match {
          case Some(concept) => concept.id + "," + groundingId + "," + concept.name + "," + concept.synonyms.map(_.toString) + "," + (concept.id == groundingId).toString
          case None => "none" + text
        }

      val this_preds = predictions.map {
        _.productIterator.map(_.toString)
      }.mkString("\n")
      println(this_preds)
      predictions.foreach(println)
      predictions1.foreach(println)
      val accuracy = predictions.count(identity).floatValue() / predictions.length
      println(accuracy)
      accuracy should be >= .7f
    }
  }


}
