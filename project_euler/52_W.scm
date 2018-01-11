(defn sorted-digits (n)
    (~> n digits sort))

(defn matches? (n)
      (let ((ds (sorted-digits n)))
        (and
          (= (sorted-digits (* 2 n)) ds)
          (= (sorted-digits (* 3 n)) ds)
          (= (sorted-digits (* 4 n)) ds)
          (= (sorted-digits (* 5 n)) ds)
          (= (sorted-digits (* 6 n)) ds))))

(defn loop (from)
  (println "from = " from)
  (if (matches? from)
      from
      (loop (inc from))))

(println "Solution: " (loop 1))
