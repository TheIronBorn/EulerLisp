; Solved 6.1

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
      (let (ds (digits10 n))
        (= ds (reverse ds))))
(defn palindromic2? (n)
      (let (ds (digits2 n))
        (= ds (reverse ds))))

(defn palindromic? (n)
      (and (palindromic2? n) (palindromic10? n)))

(defn loop (from to acc)
      (if (>= from to)
        acc
        (do 
          (println "Palindromic prime: " from)
          (if (palindromic? from)
              (loop (inc from) to (+ acc from))
              (loop (inc from) to acc)))))

(println "Solution: " (loop 1 1000000 0))
