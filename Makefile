bin_dir := bin
src_dir := src

src_files := $(wildcard $(src_dir)/**/*.java)
bin_files := $(patsubst $(src_dir)/%.java,$(bin_dir)/%.class,$(src_files))

jar := Regalis.jar
main := view/Regalis

compile_flags := 

all: $(jar)

run: $(jar)
	java -jar $<

$(jar): $(bin_files)
	jar -cfe $@ $(main) -C $(bin_dir) .

$(bin_dir)/%.class: $(src_dir)/%.java
	mkdir -p $(bin_dir)
	javac $(compile_flags) $< -d $(bin_dir) -sourcepath $(src_dir)

clean:
	rm -rf $(bin_dir)

.PHONY: all run clean

