; Solved 6.1

(defn digits (n)
      (if (= n 0)
          '()
          (cons (% n 10)
                (digits (/ n 10)))))

(defn digit-power? (n)
      (= n
         (sum (map (fn (x) (pow x 5)) (digits n)))))

(defn loop (from to acc)
      (if (> from to)
        acc
        (do 
          (if (digit-power? from)
              (loop (inc from) to (+ acc from))
              (loop (inc from) to acc)))))

; 7 * 9^5 has 6 digits,
; so 6 * 9^5 is the biggest number that could be valid
(def max-n (* 6 (pow 9 5)))

; 1^5 is not a sum
(println (loop 2 max-n 0))
