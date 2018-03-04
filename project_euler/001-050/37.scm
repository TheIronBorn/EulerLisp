; Solved 6.1.2018

(defn truncations (n)
  (cond
    [(< n 10) (list)]
    [(< n 100) (list (div n 10) (% n 10))]
    [(< n 1000)
     (list
       (div n 10) (div n 100)
       (% n 10) (% n 100))]
    [(< n 10000)
     (list
       (div n 10) (div n 100) (div n 1000)
       (% n 10) (% n 100) (% n 1000))]
    [(< n 100000)
     (list
       (div n 10) (div n 100) (div n 1000) (div n 10000)
       (% n 10) (% n 100) (% n 1000) (% n 10000))]
    [(< n 1000000)
     (list
       (div n 10) (div n 100) (div n 1000) (div n 10000) (div n 100000)
       (% n 10) (% n 100) (% n 1000) (% n 10000) (% n 100000))]
    [else (println "Error, number is to big")]))

(defn truncatable-prime? (n)
  (and (prime? n)
       (all? prime? (truncations n))))

(~> (step-stream 11 2)
    (stream-select truncatable-prime?)
    (stream-take 11)
    list-sum
    solution)
