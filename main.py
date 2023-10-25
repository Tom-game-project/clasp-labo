import base64
import os

def main():
    print("hello world")



def make_inline_wasm(wasm_path):
    # wasmの本体をbase64にしてjsに埋め込むための関数
    with open(wasm_path,mode="rb")as f:
        data=f.read()
    base64data = base64.b64encode(data)
    code = f"""
        const WASMBASE64="{base64data.decode("utf-8")}";
    """
    with open("_wasm.js",mode="w")as f:
        f.write(code)



if __name__=="__main__":
    make_inline_wasm(
        os.path.join("gast","pkg","gast_bg.wasm")
    )