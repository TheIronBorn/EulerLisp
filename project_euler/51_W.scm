
(defn replace (n i d)
  (let (p1 (pow 10 i))
  (+
    (% n p1)
    (- n (% n (* 10 p1)))
    (* d p1))))

; It can't be the last digit,
; otherwise some of the numbers in the 8-number family
; would be even
(defn replacements (n)
  (let (len (length (digits n)))
    (flatmap (fn (a)
                 (map (fn (b) (list a b))
                      (range 1 (dec a))))
             (range 2 (dec len)))))

(println (replacements 12345))
