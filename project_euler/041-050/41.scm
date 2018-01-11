; Solved 8.1

(defn permutation (index candidates acc)
      (if (empty? candidates)
          acc
      (let* (len (length candidates)
             f (fac (dec len))
             first (/ index f))
          (permutation
            (% index f)
            (delete-nth first candidates)
            (+ 
              (nth first candidates)
              (* 10 acc))))))

(defn loop (range from to result)
      (if (>= from to)
          result
          (let (perm (permutation from range 0))
            (if (prime? perm)
              (loop range (inc from) to (max perm result))
              (loop range (inc from) to result)
            ))))

(defn solve (n)
  (loop (range 1 n) 0 (fac n) 0))

; 1 + ... + n = 
;   1 -> 1, trivial
;   2 -> 3, div by 3
;   3 -> 6, div by 3
;   4 -> 10
;   5 -> 15, div by 3
;   6 -> 21, div by 3
;   7 -> 28
;   8 -> 36, div by 3
;   9 -> 45, div by 3
;
; A number is divisible by 3
; if the sum of its digits is divisible by 3
; => can't be prime

(println "Solution: " (max (solve 4) (solve 7)))
