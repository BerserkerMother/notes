import os
import logging

from flask import Flask, jsonify, request

from embedding import cosine_sim
from vector_db import VectorDB

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger("flask")

db = VectorDB(os.environ.get("VECTOR_DB_CONN_STRING"))

app = Flask("ai_engine_api")


@app.route("/")
def home():
    return "embedding engine is ready! is this magic?"


@app.route("/add", methods=["POST"])
def add():
    data = request.get_json()
    id = data["id"]
    vec = data["vec"]
    db.add_emb(id, vec)
    return "Sucess"


@app.route("/remove")
def remove():
    data = request.get_json()
    id = data["id"]

    db.remove_emb(id)
    return "Sucess"


@app.route("/search")
def search():
    data = request.get_json()
    vec = data["vec"]

    ids, embds = db.get_embds()
    indicies, scores = cosine_sim(vec, embds)

    resp = {ids[index]: scores[i] for i, index in enumerate(indicies)}
    return jsonify(resp)


app.run(debug=True, host="0.0.0.0")
