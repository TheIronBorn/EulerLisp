; Solved: 22.1.18

(def candidates
     (~> "./project_euler/input-files/98.txt"
         file-read
         (string-split ",")
         (map &(~> &1
                   string->chars
                   (select char-alphabetic?)))))

(defn anagram? (a b)
  (and
    (= (length a) (length b))
    (!= a b)
    (= (sort a) (sort b))))

(def max-len (reduce &(max (length &1) &2)       0 candidates))
(def min-len (reduce &(min (length &1) &2) max-len candidates))

(def buckets
  (~> (range min-len max-len)
      (map (fn (len)
               (select &(= (length &1) len) candidates)))))

(defn anagram-pairs (candidates (acc '()))
  (if (nil? candidates)
      acc
      (let ([a (fst candidates)]
            [rest (rst candidates)])
        (anagram-pairs
          rest
          (append
            acc
            (map &(list a &1)
                 (select &(anagram? a &1) rest)))))))

(def all-pairs
  (flatmap anagram-pairs buckets))

(defn replace (elem replacement arr)
  (cond
    [(nil? arr) '()]
    [(= (fst arr) elem)
      (cons replacement (replace elem replacement (rst arr)))]
    [else
      (cons (fst arr) (replace elem replacement (rst arr)))]))

(defn number-letters (word)
  (let ([letters (~> word sort uniq)])
    (rst
      (reduce 
        (fn (letter acc)
            (let ([idx (fst acc)]
                  [word (rst acc)])
              (cons (inc idx)
                    (replace letter idx word))))
        (cons 0 word)
        letters))))

(def number-pairs
  (map &(map number-letters &1) all-pairs))

(defn replacements (last len)
  (if (= len 1)
      (map list (range 0 (dec last)))
      (flatmap
        (fn (x)
            (map
              &(cons x &1)
              (replacements x (dec len))))
        (range (dec len) (dec last)))))

(defn all-replacements (len)
    (replacements 10 (inc len)))

(defn apply-replacement (replacement arr)
  (map &(nth &1 replacement) arr))

(defn square-replacement? (a b)
  (and
    (!= 0 (fst a))
    (!= 0 (fst b))
    (square? (digits->number (reverse a)))
    (square? (digits->number (reverse b)))))

(defn has-square-replacement? (rep a b)
  (~> (permutations~ rep)
      (select~ 
        &(square-replacement?
           (apply-replacement &1 a)
           (apply-replacement &1 b)))
      (map~ &(list
              &1
              (apply-replacement &1 a)
              (apply-replacement &1 b)))
      collect))

(defn square-replacements (a b)
  (let ([len (reduce max 0 a)])
    (let ([replacements (all-replacements len)])
      (flatmap 
        &(has-square-replacement? &1 a b)
        replacements))))

(def number-pairs_
     (list
       (list
         (number-letters (list #\R #\A #\C #\E))
         (number-letters (list #\C #\A #\R #\E))
     )))

(~>
  number-pairs
  (flatmap (fn (np) (square-replacements (fst np) (frst np))))
  println)
