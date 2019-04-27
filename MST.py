import sys
s=sys.stdin.read()
#read the input
s = s.strip()
#remove meaningless characters at the beginning and in the end
s = s.replace('\r', '\n')
s = s.replace('\t', ' ')
s = s.replace('\n\n', '\n').replace('\n\n', '\n').replace('\n\n', '\n').replace('\n\n', '\n')
#replace all possible kinds of useless characters with '\n' and ' '
#take '\n' as a mark
s_data = s.split('\n')
#separate each node
i = 0
while i < len(s_data):
	if '#' in s_data[i]:
		del(s_data[i])
		i = i - 1 
	i = i + 1
#remove the annotation line
data = []
#to store each edge's information
nodes = []
#to store nodes
for i in s_data:
	t = i.split(' ')
	try:
		int(t[2])
	except:
		exit(11)
	if len(t) != 3 or not int(t[2])  > 0:
		exit(11)
#to ensure the number of information is right(3)
#to ensure the weight is greater than 0
#to find three part of the information
	tmp=[t[0],t[1],int(t[2])]
#they are start node end node and weight
	data.append(tmp)
#to t=store the information in nested list 
	nodes.append(t[0])
	nodes.append(t[1])
#to store node names in list
nodes = list(set(nodes))
#remove repetitive node names
nodes = sorted(nodes)
#to sort nodes in lexicographical order
n = len(nodes)
#record the number of nodes
L = [i for i in range(n)]
#value in L represents tree name in number
#each index reresents a node
for i in data:
	i[0] = nodes.index(i[0])
	i[1] = nodes.index(i[1])
#to replace the node name with the index in lexicographical order
	if i[1] < i[0]:
		i[1], i[0] = i[0],i[1]
#to ensure that every two nodes of a edge is sorted in lexicographical order
class Graph:
	def __init__(self, data):
		self.data = data
#data stores edges and weight
	def father(self,L,i):
		if i != L[i]:
			L[i] = self.father(L, L[L[i]])
		return L[i]
#to find which tree does one node belong to
	def join(self,L, i, j):
		i = self.father(L, i)
		j = self.father(L, j)
		L[i] = j
#once i connect two nodes,we connect the two trees that they belong to
	def mst(self,L):
		trees = []
		for i,j,k in self.data:
			if self.father(L, i) != self.father(L, j):
				tmp = sorted([i,j]) + [k]
				if tmp not in trees:
					trees.append(tmp)
				self.join(L, i, j)
		return sorted(trees)
#to ensure the finnal edges are soreted by the lexicographical order
#if two nodes are avalible to connect,we connect them,then connect the two trees 
class Edge:
	def __init__(self, data):
		self.data = sorted(data, key = lambda x:x[2])
		#sorted lists in a list

#we sort edges by the weight 
a = Edge(data)
#instantiation
#data have nodes and weight,nodes are represented by the index of lexicographical order
b = a.data
#to sort each node's data by its weight
c = Graph(b)
#instantiation
d=c.mst(L)
#d carries the final form we want,but represented in index

for i,j,k in d:
	tmp = nodes[i] + ' ' + nodes[j] + ' ' + str(k)
#replace index with its node name
	print(tmp)
#print the finnal answer in the required forms
