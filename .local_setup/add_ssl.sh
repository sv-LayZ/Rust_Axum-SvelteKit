HEADER='\033[1;92m[Script]\033[0m'
DIR=$(dirname "$0")
echo -e "$HEADER Creating SSL Certificate for HTTPS"

echo "Creating virtual environment for Python"
python -m venv $DIR/.venv
$DIR/.venv/bin/pip install pexpect -q --no-input
$DIR/.venv/bin/python $DIR/cert.py
