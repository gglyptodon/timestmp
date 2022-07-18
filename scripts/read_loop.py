import datetime
import sys
def main():
    try:
       for line in iter(sys.stdin.readline, b''):
          if line:
              print("read @ {}:\n{}".format(datetime.datetime.now(), line))
    except KeyboardInterrupt:
       sys.stdout.flush()
       pass

if __name__=="__main__":
    main()
