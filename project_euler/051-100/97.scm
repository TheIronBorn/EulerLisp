; Solved 11.1.2018

(defn last-10 (n) (% n 10_000_000_000))

(~>
  (modexp 2 7830457 10_000_000_000)
  (* 28433)
  inc
  last-10
  solution)
