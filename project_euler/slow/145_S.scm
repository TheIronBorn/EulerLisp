; Solved: 26.1.18
; Time: 15:58
; Solution: 608720

(defn reverse-number (n nr max-div (carry 0))
      (if (zero? n)
          #t
          (let ([next 
                  (+ (% n 10)
                     (% (div nr max-div) 10)
                     carry)
                  ])
            (if (odd? (% next 10))
                (reverse-number (div n 10)
                                nr
                                (div max-div 10)
                                (div next 10))
                 #f))))

; (defn reversible? (n)
;   (if (zero? (% n 10))
;     #f
;     (reverse-number n n (pow 10 (floor (log10 n))))))

(defn add-rev-all-odd? (a b (carry 0))
  (if (nil? a)
    #t
    (let ([next (+ (fst a) (fst b) carry)])
      (if (= 1 (% next 2))
        (add-rev-all-odd? (rst a) (rst b) (div next 10))
        #f))))

(defn reversible? (n)
    (add-rev-all-odd? n (reverse n)))

(~>
  (range~ 1 100000000)
  (select~ &(< 0 (% &1 10)))
  (map~ number->digits)
  (count~ reversible?)
  (println "Solution: "))
