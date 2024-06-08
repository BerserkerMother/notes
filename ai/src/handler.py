from flask import Flask


app = Flask("ai_engine_api")

@app.route("/")
def home():
    return "embedding engine is ready!"

app.run(debug=True, host="0.0.0.0")
