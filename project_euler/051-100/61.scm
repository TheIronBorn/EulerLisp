; Solved 22.1.2018

(defn valid? (n) (not (zero? (% (div n 10) 10))))

(defn find-four-digit (f)
  (defn inner (n acc)
    (let ([next (f n)])
      (cond
        [(< next 1000) (inner (inc n) acc)]
        [(> next 9999)
         (~> acc
             (select valid?)
             (map &(divmod &1 100)))]
        [else (inner (inc n) (cons next acc))])))
  (inner 1 '()))

; Start w/ octagonal because that has the fewest elements
(def numbers
     (map find-four-digit
          (list octagonal heptagonal hexagonal pentagonal square triangular)))

(defn find-sequences (start end lists)
  (if (= 1 (length lists))
      (~> (fst lists)
          (select &(= &1 (cons start end)))
          (map list))
      (~> (fst lists)
          (select &(= (fst &1) start))
          (flatmap
            (fn (x)
                (map &(cons x &1)
                  (find-sequences (rst x) end (rst lists))))))))

(defn sequences (first rest)
     (flatmap
       (fn (x)
         (map &(cons x &1)
               (find-sequences (rst x) (fst x) rest)))
       first))

(~> (rst numbers)
    permutations
    (flatmap &(sequences (fst numbers) &1))
    fst
    (reduce-sum &(+ (* 100 (fst &1)) (rst &1)))
    solution)
