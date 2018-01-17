; Solved 1.1
; Changes:
;  * add `str` function (like in clojure)
;  * make `length` work for strings

(def single '("one" "two" "three" "four" "five" "six" "seven" "eight" "nine"))
(def teens '("ten" "eleven" "twelve" "thirteen" "fourteen" "fifteen" "sixteen"
            "seventeen" "eighteen" "nineteen"))
(def tens '("twenty" "thirty" "forty" "fifty" "sixty" "seventy" "eighty" "ninety"))

(defn format-ten (n)
    (cond
      (< n 10) (nth (dec n) single)
      (< n 20) (nth (- n 10) teens)
      else (if (zero? (% n 10))
             (nth (- (div n 10) 2) tens)
             (str
               (nth (- (div n 10) 2) tens)
               (nth (dec (% n 10)) single)))))

(defn format-hundred (n)
      (if (zero? (% n 100))
        (str
          (nth (dec (div n 100)) single)
          "hundred")
        (str
          (nth (dec (div n 100)) single)
          "hundred"
          "and"
          (format-ten (% n 100)))
        ))

(defn format (n)
      (cond
        (< n 100) (format-ten n)
        (< n 1000) (format-hundred n)
        else "onethousand"))

(defn solve ((from 1) (acc 0))
      (if (> from 1000)
        acc
        (solve
          (inc from)
          (+ acc
             (length (format from))))))

(println "Solution: " (solve))
