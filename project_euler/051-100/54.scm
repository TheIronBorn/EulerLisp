; Solved 22.1.2018

(defn value (cv)
      (cond
        [(char-numeric? cv) (char->digit cv)]
        [(= cv #\T) 10]
        [(= cv #\J) 11]
        [(= cv #\Q) 12]
        [(= cv #\K) 13]
        [(= cv #\A) 14]))

(defn parse-card (card)
      (let ([cs (string->chars card)])
        (pair (value (fst cs)) (frst cs))))

(defn parse-hand (hand)
      (map parse-card (words hand)))

(defn split-hands (hands (n 5) (acc '()))
      (if (zero? n)
        (list acc hands)
        (split-hands (rst hands)
                     (dec n)
                     (cons (fst hands) acc))))

(def all-suits (list #\H #\C #\D #\S))
(defn count-suits (suits)
      (~>
        all-suits
        (map &(pair (count (fn (o) (= o &1)) suits) &1))
        sort
        reverse))

(def all-values (range 1 14))
(defn count-values (vals)
      (~>
        all-values
        (map &(pair (count (fn (o) (= o &1)) vals) &1))
        sort
        reverse))

(defn consecutive?_ (vals last)
      (cond
        [(nil? vals) #t]
        [(= (fst vals) (dec last))
         (consecutive?_ (rst vals) (fst vals))]
        [else #f]))
(defn consecutive? (vals)
      (consecutive?_ (rst vals) (fst vals)))

(defn classify (hand)
      (let ([vals (~> hand (map fst) sort reverse)]
            [suits (map rst hand)])
        (let ([val-counts (~> vals count-values)]
              [suit-counts (~> suits count-suits)])
          (cond
            [(and (= 5 (ffst suit-counts))
                  (consecutive? vals))
             (+ 80000 (fst vals))]
            [(= 4 (ffst val-counts))
             (+ 70000
                (rfst val-counts))]
            [(and (= 3 (ffst val-counts))
                  (= 2 (ffrst val-counts)))
             (+ 60000
                (* 100 (rfst val-counts))
                (rfrst val-counts))]
            [(= 5 (ffst suit-counts)) 50000]
            [(consecutive? vals) 40000]
            [(= 3 (ffst val-counts))
             (+ 30000
                (rfst val-counts))]
            [(and (= 2 (ffst val-counts))
                  (= 2 (ffrst val-counts)))
             (+ 20000
                (* 100 (rfst val-counts))
                (rfrst val-counts))]
            [(= 2 (ffst val-counts))
             (+ 10000
                (rfst val-counts))]
            [else 0]
            ))))

(defn winner-highest? (vs1 vs2)
      (cond
        [(nil? vs1) #t]
        [(> (fst vs1) (fst vs2)) #t]
        [(< (fst vs1) (fst vs2)) #f]
        [else (winner-highest? (rst vs1) (rst vs2))]))

(defn winner? (line)
      (let* ([hands (~> line parse-hand split-hands)]
             [hand1 (fst hands)]
             [hand2 (frst hands)])
        (let ([class1 (classify hand1)]
              [class2 (classify hand2)])
          (cond
            [(> class1 class2) #t]
            [(= class1 class2)
             (winner-highest?
               (~> hand1 (map fst) sort reverse)
               (~> hand2 (map fst) sort reverse))]
            [else #f]))))

(~> "./project_euler/input-files/54.txt"
    file-read
    lines
    (reject empty?)
    (count winner?)
    solution)
