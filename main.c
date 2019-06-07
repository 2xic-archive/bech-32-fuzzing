#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>
#include "c/segwit_addr.c"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h> 
#include <unistd.h>

extern int bech32_decode_rust(const char *str);
int bech32_decode(
    char *hrp,
    uint8_t *data,
    size_t *data_len,
    const char *input
);

int bech32_decode_patch(
    char *hrp,
    uint8_t *data,
    size_t *data_len,
    const char *input
);

int main(int argc, char** argv) {
	if(argc == 2 || argc == 3){
		if( access(argv[1], F_OK ) != -1 ) {
			FILE *f = fopen(argv[1], "rb");
			fseek(f, 0, SEEK_END);
			long fsize = ftell(f);
			fseek(f, 0, SEEK_SET); 

			char *string = malloc(fsize + 1);
			fread(string, 1, fsize, f);
			fclose(f);

			string[fsize] = 0;

			uint8_t data[82];
			char hrp[84];
			size_t data_len;

			uint8_t data_p[82];
			char hrp_p[84];
			size_t data_len_p;
			
			int i = bech32_decode_rust(string);
			int j = bech32_decode(hrp, data, &data_len, string);

			// testing
			if(argc == 3){
				int k = bech32_decode_patch(hrp_p, data_p, &data_len_p, string);
				printf("%i %i %i\n", j, k, i);
			}
			free(string);
			assert(i == j);
		}else{
			printf("check path\n");
		}
	}
}
