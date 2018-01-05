(defn digits (n)
      (if (= n 0)
          '()
          (cons (% n 10)
                (digits (/ n 10)))))

(defn digit-power? (n)
      (= n
         (sum (map (fn (x) (pow x 4)) (digits n)))))

(println (digit-power? 1634))

(defn loop (from to acc)
      (if (> from to)
        acc
        (do 
          (println from)
          (if (digit-power? from)
              (loop (inc from) to (+ acc from))
              (loop (inc from) to acc)))))


(def max-n (* 6 (pow 9 5)))
(println (loop 2 max-n 0))
