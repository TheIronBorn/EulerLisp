; Solved 11.1

(defn sorted-digits (n)
    (~> n digits sort))

(defn matches? (n)
      (= (sorted-digits n)
         (sorted-digits (* 2 n))
         (sorted-digits (* 3 n))
         (sorted-digits (* 4 n))
         (sorted-digits (* 5 n))
         (sorted-digits (* 6 n))))

(~>
  (step~ 1)
  (select~ matches?)
  (nth~ 0)
  (println "Solution: "))
