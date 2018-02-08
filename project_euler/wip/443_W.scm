(defn g (n (from 4) (acc 13))
  (if (= from n)
      acc
      (g n
         (inc from)
         (+ acc (gcd acc (inc from))))))

(println (g 1000_000_000))
