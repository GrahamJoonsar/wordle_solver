import requests
import time
_wait = 0.5

def get_freq(term):
    response = None
    while True:
        try:
            response = requests.get('https://api.datamuse.com/words?sp='+term+'&md=f&max=1').json()
        except:
            print('Could not get response. Sleep and retry...')
            time.sleep(_wait)
            continue
        break;
    freq = 0.0 if len(response)==0 else float(response[0]['tags'][0][2:])
    return freq

count = 0
with open("words.txt") as fp:
    with open("word_freq.txt", "w") as wp:
        for line in fp:
            word = line.strip()
            freq = get_freq(word)
            wp.write("{} {}\n".format(word, str(freq)))
            if count % 100 == 0:
                print("Word: {}\tFrequency: {}".format(word, freq))
            count += 1