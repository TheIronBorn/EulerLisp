; Solved 8.1

(def ps (list->vector (permutations (range 0 9))))

(defn loop (from to result)
      (if (>= from to)
        result
        (let ((perm (string->number (apply str (vector-ref ps from)))))
          (if (substring-divisible? perm)
            (loop (inc from) to (+ perm result))
            (loop (inc from) to result)
            ))))

(defn substring (n offset)
      (% (/ n (pow 10 (- 7 offset))) 1000))

(defn substring-divisible? (n)
      (and
        (divides? 17 (substring n 7))
        (divides? 13 (substring n 6))
        (divides? 11 (substring n 5))
        (divides?  7 (substring n 4))
        (divides?  5 (substring n 3))
        (divides?  3 (substring n 2))
        (divides?  2 (substring n 1))))

(println "Solution: " (loop 0 (fac 10) 0))
