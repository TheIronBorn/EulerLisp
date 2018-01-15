; Solved 6.1

(defn digits2 (n)
      (if (= n 0)
          '()
          (cons (% n 2)
                (digits2 (/ n 2)))))

(defn palindromic10? (n)
      (let (ds (digits n))
        (= ds (reverse ds))))
(defn palindromic2? (n)
      (let (ds (digits2 n))
        (= ds (reverse ds))))

(defn palindromic? (n)
      (and (palindromic2? n) (palindromic10? n)))

(defn solve ((from 1) (acc 0))
      (if (>= from 1000000)
        acc
        (if (palindromic? from)
          (do 
            (println "Palindromic: " from)
            (solve (inc from) (+ acc from)))
          (solve (inc from) acc))))

(println "Solution: " (solve))
