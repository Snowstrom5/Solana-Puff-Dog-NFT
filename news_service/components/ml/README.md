
# ML

A program used to prepare data used in training an OpenAI fine-tuned model. 
The fine-tuned model is used to classify climate related articles.

OpenAI example notebook: https://github.com/openai/openai-cookbook/blob/main/examples/Fine-tuned_classification.ipynb


### Data Preparation
```
openai tools fine_tunes.prepare_data -f news_feed_urls.jsonl
export OPENAI_API_KEY=<open_api_key>
```

### Fine-tuning
```
openai api fine_tunes.create -t "news_feed_urls_prepared_train.jsonl" -v "news_feed_urls_prepared_valid.jsonl" --compute_classification_metrics --classification_positive_class " 1"
```
Note prompt has to end with the indicator string ` \n\n###\n\n`
```
Created fine-tune: ft-fqXnErmYT0dAuVR4Urok8hEe
Uploaded model: curie:ft-personal-2022-10-14-18-16-36
Uploaded result file: file-66KckvW4bbg4qcfy1LPR7Vmi
```

### Results
```
openai api fine_tunes.results -i ft-fqXnErmYT0dAuVR4Urok8hEe > result.csv
```
