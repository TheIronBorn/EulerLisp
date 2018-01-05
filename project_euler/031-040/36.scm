(defn digits10 (n)
      (if (= n 0)
          '()
          (cons (% n 10)
                (digits10 (/ n 10)))))

(defn digits2 (n)
      (if (= n 0)
          '()
          (cons (% n 2)
                (digits2 (/ n 2)))))

(defn palindromic10? (n)
      (let ((digits (digits10 n)))
        (= digits (reverse digits))))
(defn palindromic2? (n)
      (let ((digits (digits2 n)))
        (= digits (reverse digits))))

(defn palindromic? (n)
      (and (palindromic2? n) (palindromic10? n)))

(defn loop (from to acc)
      (if (>= from to)
        acc
        (do 
          (println from)
          (if (palindromic? from)
              (loop (inc from) to (+ acc from))
              (loop (inc from) to acc)))))

(println (loop 1 1000000 0))
