""" Temporary test script for development This will change to unit tests and example usage script """

from skema.text_reading.mention_linking.gromet_linker import align_and_link

from automates.utils.fold import dictionary_to_gromet_json, del_nulls

if __name__ == "__main__":
	gromet_path = "data/gromet/CHIME_SIR--Gromet-FN-auto.json"
	comments_path = "data/comments/CHIME_SIR.json"
	extractions_path = 'data/extractions/CHIME_SIR.json'

	# gromet_path = "data/gromet/chime_penn--Gromet-FN-auto.json"
	# comments_path = "data/comments/CHIME_full_penn.json"
	# extractions_path = 'data/extractions/CHIME_SIR.json'

	# gromet_path = "data/gromet/bucky_simplified_v1--Gromet-FN-auto.json"
	# comments_path = "data/comments/BUCKY.json"
	# extractions_path = 'data/extractions/BUCKY.json'

	embeddings_path = "/data/covid_comments_models/xdd_covid_19_1x_word2vec/alternate/embeddings.kv"
	enriched_gromet = align_and_link(gromet_path, comments_path, extractions_path, embeddings_path, debug=True)

	# Save gromet file with the new metadata aligned
	with open("test.json", 'w') as f:
		f.write(dictionary_to_gromet_json(del_nulls(enriched_gromet.to_dict())))