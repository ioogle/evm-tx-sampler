.PHONY: run-st, run-api

run-st:
	streamlit run app/app.py

run-api:
	cargo run