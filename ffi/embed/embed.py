from ctypes import cdll

embedlib = cdll.LoadLibrary("target/release/libembed.dylib")

embedlib.process()

print("done!")
