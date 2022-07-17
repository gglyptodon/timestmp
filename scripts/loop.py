import time
import datetime

def main():
    while True:
        print("hello! @ {}".format(datetime.datetime.now()))
        time.sleep(5)


if __name__=="__main__":
    main()
