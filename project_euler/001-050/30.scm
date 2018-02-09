; Solved 6.1.2018

(defn digit-power? (n)
      (= n
         (reduce-sum
           &(pow &1 5)
           (number->digits n))))

; 7 * 9^5 has 6 digits,
; so 6 * 9^5 is the biggest number that could be valid
;
; 1^5 is not a sum
(~>
  (range~ 2 (* 6 (pow 9 5)))
  (select~ digit-power?)
  sum~
  solution)
