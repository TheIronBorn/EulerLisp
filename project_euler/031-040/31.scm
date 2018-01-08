; Solved 6.1

(defn coin-sums (n coins)
      (if (= (length coins) 1)
          1
          (sum
            (map (fn (x)
                     (coin-sums (- n (* x (fst coins)))
                                (rst coins)))
                 (range 0 (/ n (fst coins)))))))

(println (coin-sums 200 (list 200 100 50 20 10 5 2 1)))
