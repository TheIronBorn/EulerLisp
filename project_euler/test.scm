(defn loop (cur)
  (do
    (println cur)
    (loop (inc cur))))

(loop 1)
