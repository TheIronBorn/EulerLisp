; Solved: 22.2.2018

(def cc-cards
  (vector
    '(go-abs . 0) '(go-abs . 10)
    '(0 . 0) '(0 . 0) '(0 . 0) '(0 . 0)
    '(0 . 0) '(0 . 0) '(0 . 0) '(0 . 0)
    '(0 . 0) '(0 . 0) '(0 . 0) '(0 . 0)
    '(0 . 0) '(0 . 0)))

(def ch-cards
  (vector
    '(go-abs . 0) '(go-abs . 10) '(go-abs . 11)
    '(go-abs . 24) '(go-abs . 39) '(go-abs . 5)
    '(go-next . r) '(go-next . r) '(go-next . u)
    '(go-back . 3)
    '(0 . 0) '(0 . 0) '(0 . 0) '(0 . 0)
    '(0 . 0) '(0 . 0)))

(def cc-index -1)
(def ch-index -1)

(vector-shuffle! cc-cards)
(vector-shuffle! ch-cards)

(defn get-cc ()
  (set! cc-index
        (% (inc cc-index) (vector-length cc-cards)))
  (vector-ref cc-cards cc-index))
(defn get-ch ()
  (set! ch-index
        (% (inc ch-index) (vector-length ch-cards)))
  (vector-ref ch-cards ch-index))

(defn roll ()
  (let ([a (rand 1 4)] [b (rand 1 4)])
    (cons (+ a b) (= a b))))

(def freqs (make-vector 40 0))
(def position 0)
(def doubles 0)

(defn increase-freq (pos)
  (vector-set!
    freqs
    pos
    (inc (vector-ref freqs pos))))

(defn go-to (pos)
  (set! position pos)
  (increase-freq pos))

(defn next-r ()
  (cond
    [{position < 5} 5]
    [{position < 15} 15]
    [{position < 25} 25]
    [{position < 35} 35]
    [else 5]))

(defn next-u ()
  (cond
    [{position < 12} 12]
    [{position < 28} 28]
    [else 12]))

(defn handle-cc ()
  (let ([card (get-cc)])
    (case (fst card)
      ['go-abs (go-to (rst card))]
      [else (increase-freq position)])))

(defn handle-ch ()
  (let ([card (get-ch)])
    (case (fst card)
      ['go-abs (go-to (rst card))]
      ['go-next
       (if (= (rst card) 'r)
         (go-to (next-r))
         (go-to (next-u)))]
      ['go-back
       (do
         (set! position (% {position - 3} 40))
         (if {position = 33}
           (handle-cc)
           (increase-freq position)))]
      [else (increase-freq position)])))

(defn make-step ()
  (let ([r (roll)])
    (if (rst r)
        (set! doubles (inc doubles))
        (set! doubles 0))
    (if {doubles = 3}
        (do
          (set! doubles 0)
          (go-to 10))
        (let ([new (% (+ position (fst r)) 40)])
          (set! position new)
          (cond
            [{new = 30} (go-to 10)] ; G2J
            [(or {new = 2} {new = 17} {new = 33})
             (handle-cc)]
            [(or {new = 7} {new = 22} {new = 36})
             (handle-ch)]
            [else (increase-freq new)])))))

(defn make-steps (n)
  (when {n > 0}
    (make-step)
    (make-steps (dec n))))

(vector-set! freqs 0 1)
(make-steps 500_000)

(~>
  (map* cons
    (list 
      (vector->list freqs)
      (range 0 40)))
  sort
  reverse
  (take 3)
  (map rst)
  (apply str)
  solution)
