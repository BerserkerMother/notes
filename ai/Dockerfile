FROM python:3.10 AS base

WORKDIR /app

# developing with exteremly bad net
RUN pip install torch --index-url https://download.pytorch.org/whl/cpu

FROM base AS torch-downloader

# Install Python dependencies
RUN pip install flask pymongo python-dotenv numpy transformers tokenizers sentence_transformers

FROM torch-downloader AS model-downloader
# Download the Hugging Face model using transformers
RUN python -c "from transformers import AutoModel; AutoModel.from_pretrained('sentence-transformers/all-MiniLM-L6-v2')"

EXPOSE 5000

CMD ["python", "handler.py"]

