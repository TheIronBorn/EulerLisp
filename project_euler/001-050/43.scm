; Solved 8.1.2018

(defn substring (n offset)
      (% (div n (pow 10 (- 7 offset))) 1000))

(defn substring-divisible? (n)
      (and
        (divides? 17 (substring n 7))
        (divides? 13 (substring n 6))
        (divides? 11 (substring n 5))
        (divides?  7 (substring n 4))
        (divides?  5 (substring n 3))
        (divides?  3 (substring n 2))
        (divides?  2 (substring n 1))))

(~>
  (range 0 9)
  permutations~
  (map~ digits->number)
  (select~ substring-divisible?)
  sum~
  solution)
