; Solved: 26.1.18
; Time: 15:58
; Solution: 608720

(defn reversible? (n nr max-div (carry 0))
      (if (zero? n)
          #t
          (let ([next 
                  (+ (% n 10)
                     (% (div nr max-div) 10)
                     carry)
                  ])
            (if (odd? (% next 10))
                (reversible? (div n 10)
                                nr
                                (div max-div 10)
                                (div next 10))
                 #f))))

(defn count-range (n)
  (let ([from (pow 10 n)]
        [to (pow 10 (inc n))])
  (~>
    (range~ from (dec to))
    (select~ &(< 0 (% &1 10)))
    (count~ &(reversible? &1 &1 from)))))

; (println (count-range 0))
(println (count-range 1))
(println (count-range 2))
(println (count-range 3))
(println (count-range 4))
(println (count-range 5))
(println (count-range 6))
(println (count-range 7))
(println (count-range 8))

; (~>
;   ; (range~ 1 100_000_000)
;   (range~ 1 5_00_000)
;   (select~ &(< 0 (% &1 10)))
;   ; (map~ number->digits)
;   (count~ reversible?)
;   (println "Solution: "))
