#!/usr/bin/env perl

$s = '';
for my $i ( 0 .. 1609 ) {
	$s .= chr( ord('a')+rand(23));
}
$s2 = '';
for my $i ( 0 .. 1609 ) {
	if ( rand(100) > 75 ) {
		$s2 .= chr( ord('A')+rand(23));
	} else {
		$s2 .= substr($s,$i,1);
	}
}

print "$s\n";
print "$s2\n";
