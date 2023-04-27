OUTDIR="tests/expected"

[[ -d "$OUTDIR" ]] && rm -rf "$OUTDIR"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"


echo "Hello there" > $OUTDIR/hello1.txt
echo "Hello"  "there" > $OUTDIR/hello2.txt

echo "text" > $OUTDIR/write_new.txt
echo "old text" > $OUTDIR/write_add.txt
echo "new text" >> $OUTDIR/write_add.txt