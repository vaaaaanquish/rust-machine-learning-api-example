import base64
import json

try:
    import requests
except:
    print("// -----------------------\n[requirement] pip install requests\n// --------------------------")
    raise


def request_image(img):
    print('request to localhost:3000')
    res = requests.post('http://127.0.0.1:3000', data=json.dumps({'img': img}), headers={'content-type': 'application/json'})
    print('response\n', json.dumps(json.loads(res.text), indent=2))


# get sample image
print('get sample image from rust-lang.org')
sample_image_response = requests.get('http://rust-lang.org/logos/rust-logo-128x128-blk.png')
img = base64.b64encode(sample_image_response.content).decode('utf-8')

# request
request_image(img)
