; Solved: 23.2.2018

(defn make-even-palindrome (n)
  (+ (* n (pow 10 (number-of-digits n)))
     (~> n number->digits reverse digits->number)))
(defn make-odd-palindrome (n)
  (+ (* (div n 10) (pow 10 (number-of-digits n)))
     (~> n number->digits reverse digits->number)))

(def even-palindromes
  (~> (range~ 1 9_999)
      (map~ make-even-palindrome)))
(def odd-palindromes
  (~> (range~ 1 9_999)
      (map~ make-odd-palindrome)))

(def square-sums
     (~> (range~ 0 10_000)
         (map~ gauss-square-sum)
         collect
         list->vector))

(def square-range-sums (make-bitvector (pow 10 8)))
(defn square-range-sum? (n)
  (bitvector-get square-range-sums n))

(defn fill-srs (from to)
  (let ([v (- (vector-ref square-sums to)
              (vector-ref square-sums from))])
    (when {v < 100_000_000}
      (bitvector-set! square-range-sums v)
      (fill-srs from (inc to)))))

(~> (range~ 0 9998)
    (each~ &(fill-srs &1 (+ &1 2))))

(solution
  (+ (~> even-palindromes (select~ square-range-sum?) sum~)
     (~> odd-palindromes (select~ square-range-sum?) sum~)))
