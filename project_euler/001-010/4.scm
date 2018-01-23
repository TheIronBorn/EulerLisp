; Solved: 17.12.17
; Changes:
;   * implement TCO for cond

(defn solve ((a 100) (b 100) (prod 0))
      (cond
        [(> a b) (solve 100 (inc b) prod)]
        [(> b 999) prod]
        [(and (> (* a b) prod)
              (palindromic? (number->digits (* a b))))
         (solve (inc a) b (* a b))]
        [else (solve (inc a) b prod)]))

(println "Solution: " (solve))
