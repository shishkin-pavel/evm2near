### This script assumes that evm2near is already compiled

import os

contracts = [
    'calc',
    'bench',
    'Collatz',
    'echo',
    'const'
]


def compile(name: str):
    os.system(f'./evm2near test/{name}.sol -o {name}.wasm -b wasi')


def copy(name: str):
    os.system(f'cp {name}.wasm tools/benchmark/{name}.wasm')


def remove(name: str):
    os.system(f'rm tools/benchmark{name}.wasm')


def compile_contracts():
    for contract in contracts:
        compile(contract)


def copy_contracts():
    for contract in contracts:
        copy(contract)


def clean():
    for contract in contracts:
        remove(contract)


def run_bench():
    os.system('cd tools/benchmark')
    os.system('cargo run')


if __name__ == "__main__":
    compile_contracts()
    copy_contracts()
    run_bench()    
    clean()
    