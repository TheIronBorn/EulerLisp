; Solved: 28.1.18

(defn digit-sum (n) (sum (number->digits n)))

; Assuming that 64 bit signed ints are big enough,
; the maximal digit sum is limited
(def max-ds (~> 10 log2 (/ 63) ceil (* 9)))

; Maximal power of n that would fit inside a i64
(defn max-power (n) (~> n log2 (/ 63) floor))

(defn powers (n)
  (~> (range~ 2 (max-power n))
      (map~ &(pow n &1))
      (select~ &(= (digit-sum &1) n))
      collect))

(~>
  (range 2 max-ds)
  (flatmap powers)
  sort
  (nth 29)
  (println "Solution: "))
