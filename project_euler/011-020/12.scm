; Solved: 29.12.17
; Changes:
;  * add builtin `factors` method (although it seems like cheating)

(defn solve ((number 1) (n 2))
  (let (npf (length (factors number)))
    (if (> npf 500)
      number
      (solve (+ number n) (inc n)))))

(println "Solution: " (solve))
