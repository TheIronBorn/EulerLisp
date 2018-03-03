; Solved: 23.2.2018
; Time: 3:30

(defn count-ways (n)
  (~> (range~ 1 (icbrt n))
      (map~ cube)
      (count~ &(square? (- n &1)))))

(defn make-even-palindrome (n)
  (+ (* n (pow 10 (number-of-digits n)))
     (~> n number->digits reverse digits->number)))
(defn make-odd-palindrome (n)
  (+ (* (div n 10) (pow 10 (number-of-digits n)))
     (~> n number->digits reverse digits->number)))

(defn n-digit-palindromes (n)
  (let ([to (pow 10 (ceil (/ n 2)))])
    (if (even? n)
      (~> (range~ (div to 10) (dec to))
          (map~ make-even-palindrome))
      (~> (range~ (div to 10) (dec to))
          (map~ make-odd-palindrome)))))


(~> (step~ 1)
    (flatmap-stream~ n-digit-palindromes)
    (select~ &(= 4 (count-ways &1)))
    (take~ 5)
    list-sum
    solution)

