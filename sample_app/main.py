#!/usr/bin/env python3

from flask import Flask, request
app = Flask(__name__)


@app.route('/', methods=['POST', 'GET'])
def hello_world():
    if request.method == 'POST':
        command = request.form.get('command')
        return f'result: {eval(command)}'

    content = '''
    <html>
        <body>
            <form action = "/" method = "post">
            <p>Enter Command:</p>
            <p><input type = "command" name = "command" /></p>
            <p><input type = "submit" value = "submit" /></p>
            </form>   
        </body>
    </html>
    '''

    return content


if __name__ == '__main__':
    app.run(debug=True, host='0.0.0.0')
