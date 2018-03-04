; Solved 11.1.2018

(defn sorted-digits (n)
    (~> n number->digits sort))

(defn matches? (n)
      (= (sorted-digits n)
         (sorted-digits (* 2 n))
         (sorted-digits (* 3 n))
         (sorted-digits (* 4 n))
         (sorted-digits (* 5 n))
         (sorted-digits (* 6 n))))

;; Only consider numbers that will have the same number of digits
;; when multiplied by 6
(defn search-range (dgts)
  (~> (range-stream (pow 10 dgts) (div (pow 10 (inc dgts)) 6))
      (stream-select matches?)))

(~> (step-stream 1 1)
    (stream-flatmap search-range)
    (stream-select matches?)
    fst
    solution)
