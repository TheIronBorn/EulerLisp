; Solved 6.1

(defn digits2 (n (acc '()))
      (if (= n 0)
          acc
          (digits2 (div n 2)
                   (cons (% n 2) acc))))

(defn palindromic10? (n)
      (let (ds (digits n))
        (= ds (reverse ds))))
(defn palindromic2? (n)
      (let (ds (digits2 n))
        (= ds (reverse ds))))

(defn palindromic? (n)
      (and (palindromic2? n) (palindromic10? n)))

(~>
  (range~ 1 1000000)
  (select~ palindromic?)
  sum~
  (println "Solution: "))
