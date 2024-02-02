rm -rf $PATH_TO_DB
echo "Path: " $PATH_TO_DB
mkdir $PATH_TO_DB
mkdir "$PATH_TO_DB/node-1"
mkdir "$PATH_TO_DB/node-2"
mkdir "$PATH_TO_DB/node-3"
mkdir "$PATH_TO_DB/node-4"
mkdir "$PATH_TO_DB/node-5"
cd zk-node && cargo test --lib create_noir_db -- $PATH_TO_DB/node-1 \
	&& cargo test --lib create_noir_db -- $PATH_TO_DB/node-2 \
	&& cargo test --lib create_noir_db -- $PATH_TO_DB/node-3 \
	&& cargo test --lib create_noir_db -- $PATH_TO_DB/node-4 \
	&& cargo test --lib create_noir_db -- $PATH_TO_DB/node-5 \
	&& cargo test --lib create_circom_db -- $PATH_TO_DB/node-1 \
	&& cargo test --lib create_circom_db -- $PATH_TO_DB/node-2 \
	&& cargo test --lib create_circom_db -- $PATH_TO_DB/node-3 \
	&& cargo test --lib create_circom_db -- $PATH_TO_DB/node-4 \
	&& cargo test --lib create_circom_db -- $PATH_TO_DB/node-5
