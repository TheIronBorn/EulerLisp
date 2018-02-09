; Solved 6.1.2018

(defn coin-sums (n coins)
      (if (= (length coins) 1)
          1
          (reduce-sum
            &(coin-sums (- n (* &1 (fst coins)))
                        (rst coins))
             (range 0 (div n (fst coins))))))

(~> (list 200 100 50 20 10 5 2 1)
    (coin-sums 200)
    solution)
