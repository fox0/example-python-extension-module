from ctypes import cdll


def main():
    lib = cdll.LoadLibrary("target/release/libtree.so")
    lib.process()
    print("done!")


if __name__ == '__main__':
    main()
