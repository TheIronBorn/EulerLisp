; Solved 6.1

(defn digit-power? (n)
      (= n
         (sum (map (fn (x) (pow x 5)) (digits n)))))

; 7 * 9^5 has 6 digits,
; so 6 * 9^5 is the biggest number that could be valid
(def max-n (* 6 (pow 9 5)))

(defn solve (from (acc 0))
      (if (> from max-n)
        acc
        (do 
          (if (digit-power? from)
              (solve (inc from) (+ acc from))
              (solve (inc from) acc)))))


; 1^5 is not a sum
(println "Solution: " (solve 2))
